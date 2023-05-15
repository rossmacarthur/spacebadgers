import Image from 'next/image'
import Badge from './Badge'

export default function Home() {
  return (
    <main className="flex flex-col gap-8">
        <div className="bg-gray-100 p-4 rounded-md font-mono text-sm text-center">
            <span className="text-gray-400">https://</span>
            <span className="text-gray-700">honey.badgers.space</span>
            <span className="text-gray-400">/</span>
            <span>badge</span>
            <span className="text-gray-400">/</span>
            <span className="text-green-600">:label</span>
            <span className="text-gray-400">/</span>
            <span className="text-emerald-600">:status</span>
            <span className="text-gray-400">/</span>
            <span className="text-teal-600">:color</span>
        </div>
        <div className="flex flex-col gap-4 items-center">
            <h2 className="text-xl text-gray-700 font-bold">Supported Colors</h2>
            <div className="flex gap-1">
                <Badge label="color" status="blue" color="blue" />
                <Badge label="color" status="cyan" color="cyan" />
                <Badge label="color" status="green" color="green" />
                <Badge label="color" status="yellow" color="yellow" />
                <Badge label="color" status="orange" color="orange" />
                <Badge label="color" status="red" color="red" />
                <Badge label="color" status="pink" color="pink" />
                <Badge label="color" status="purple" color="purple" />
                <Badge label="color" status="gray" color="gray" />
                <Badge label="color" status="black" color="black" />
            </div>
        </div>
    </main>
  )
}
