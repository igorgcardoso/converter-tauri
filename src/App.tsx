import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';
import clsx from 'clsx';
import { useState } from 'react';

function App() {
  const [selectedFile, setSelectedFile] = useState('');
  const [resolution, setResolution] = useState('');
  const [isConverting, setIsConverting] = useState(false);
  const [done, setDone] = useState(false);

  async function openFile() {
    setDone(false);
    const result = await open({
      multiple: false,
      filters: [
        {
          name: '*',
          extensions: ['mp4', 'mkv', 'avi', 'webm'],
        },
      ],
    });

    if (result && typeof result === 'string') {
      setSelectedFile(result);
    }
  }

  async function convertFile() {
    if (selectedFile) {
      try {
        setIsConverting(true);
        await invoke('convert_file', {
          path: selectedFile,
          resolution: resolution || 'Same',
        });
        setDone(true);
      } finally {
        setIsConverting(false);
      }
    }
  }

  return (
    <div className="w-screen h-screen bg-slate-900">
      <div className="flex flex-col items-center justify-center h-full">
        <button
          className="px-4 py-2 text-white bg-blue-500 rounded-md hover:bg-blue-600"
          onClick={openFile}
        >
          Abrir arquivo
        </button>
        {selectedFile && (
          <strong
            className={clsx('mt-4 text-white text-center', {
              'text-yellow-400': isConverting,
              'text-green-500': done,
            })}
          >
            {selectedFile}
          </strong>
        )}

        <div className="flex flex-col items-center justify-center mt-4">
          <label
            htmlFor="resolution"
            className="font-bold text-white text-lg leading-relaxed"
          >
            Resolução
          </label>
          <select
            name="resolution"
            id="resolution"
            className="px-2 py-1 border border-gray-500 rounded-md"
            defaultValue={'Same'}
            onChange={(e) => setResolution(e.target.value)}
          >
            <option value="Same">Manter</option>
            <option value="Sd">480p</option>
            <option value="Hsd">600</option>
            <option value="Hd">720p</option>
            <option value="Hdd">900p</option>
          </select>
        </div>
        {!isConverting && (
          <button
            className="px-4 py-2 mt-4 text-white bg-green-500 rounded-md hover:bg-green-600"
            onClick={convertFile}
            disabled={isConverting}
          >
            Converter
          </button>
        )}
      </div>
    </div>
  );
}

export default App;
