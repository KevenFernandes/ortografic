import { invoke } from '@tauri-apps/api/core';
import clsx from 'clsx';
import { useEffect, useState } from 'react';
import './App.css';

interface RespostaVerificacao {
  status: 'correto' | 'erro';
  sugestoes: string[];
}

function App() {
  const [palavra, setPalavra] = useState<string>('');
  const [resultado, setResultado] = useState<RespostaVerificacao | null>(null);

  function verificaTexto (texto: string) {
    setPalavra(texto);
  };


  useEffect(() => {
    if(!palavra.trim()){
        setResultado(null)
        return
      }

      const timer = setInterval(async () => {
        try {
          const res = await invoke<RespostaVerificacao>('verificar_texto', {
            palavraDigitada: palavra, 
          });
          setResultado(res);
        } catch (err) {
          console.error('Erro ao verificar palavra:', err);
        }
      }, 300)

      return () => clearTimeout(timer)
  }, [palavra])
     
  return (
    <main className="flex h-svh p-4 gap-4 items-start">
      <div>
        <header className='mb-2 flex items-center gap-2'>
          <h1 className="text-xl">Ortografia</h1>
          <div className='group relative'>
            <span className='border-2 text-sm rounded-full w-5 h-5 flex items-center justify-center border-gray-500 text-gray-400 cursor-pointer'>?</span>

            <div className={clsx(
              'hidden group-hover:flex absolute',
              'left-4 top-4 rounded-tl-none',
              'bg-gray-600 p-2 w-60 rounded-lg'
              )}>
              <p className='text-sm text-gray-200'>Digite sua palavra abaixo para verificar e corrigir a ortografia com sugestões inteligentes.</p>
            </div>
          </div>
        </header>

        <div className='flex flex-col gap-2'>
            <input
              type="text"
              value={palavra}
              onChange={e => verificaTexto(e.target.value)}
              placeholder="Digite uma palavra..."
              className='px-2 py-1 w-50 text-base'
            />
            {!!palavra.length && resultado && (
              resultado.status === 'correto' ? (
                <p className='text-green-500 font-bold text-sm'>
                  <span className='text-xs'>✅</span> Palavra correta!
                </p>
              ) : (
                <p className='text-red-500 font-bold text-sm'>
                  <span className='text-xs'>❌</span> Palavra incorreta!
                </p>
              )
            )}
          </div>
      </div>


      <div className='gap-4'>
        {resultado && resultado.status !== 'correto' && (
          <div>
            <p className='text-blue-400 underline'>Sugestões:</p>
            <ul>
              {resultado.sugestoes.map((sug, index) => (
                <li key={index}>{sug}</li>
              ))}
            </ul>
          </div>
        )}
      </div>
    </main>
  );
}

export default App;
