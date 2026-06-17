import {useState} from "react";

export default function App() {

    const [step, setStep] = useState(1);
    return (
        <>
            {step == 1 && (
                <div className="min-h-screen w-full bg-white flex items-center justify-center px-6">
                    <div className="w-full max-w-4xl text-center">
                        <div className="inline-flex items-center rounded-full bg-[#ffb87b]/15 px-4 py-2 mb-6 border border-[#ffb87b]/20">
                          <span className="text-sm font-medium text-[#ffb87b]">
                            JosdorOS Quick Setup
                          </span>
                        </div>

                        <h1 className="text-5xl md:text-6xl font-bold tracking-tight text-zinc-500">
                            Configure your
                            <span className="text-[#ffb87b]"> brand new network</span>
                        </h1>

                        <p className="mt-4 text-lg text-zinc-500">
                            Start by giving your router a hostname :
                        </p>

                        <div className="mt-10">
                            <input
                                type="text"
                                placeholder="Enter hostname..."
                                className="w-full rounded-2xl border border-zinc-200 px-6 py-5 text-lg outline-none focus:border-[#ffb87b] focus:ring-4 focus:ring-[#ffb87b]/20"
                            />
                        </div>

                        <button onClick={() => setStep(2)} className="mt-6 rounded-2xl bg-[#ffb87b] px-8 py-4 text-white font-semibold shadow-md hover:scale-[1.02] transition">
                            Configure
                        </button>
                    </div>
                </div>
            )}
            {step == 2 && (
                <div className="min-h-screen w-full bg-white flex items-center justify-center px-6">
                    <div className="w-full max-w-4xl text-center">
                        <div className="inline-flex items-center rounded-full bg-[#ffb87b]/15 px-4 py-2 mb-6 border border-[#ffb87b]/20">
                          <span className="text-sm font-medium text-[#ffb87b]">
                            JosdorOS Quick Setup
                          </span>
                        </div>

                        <h1 className="text-5xl md:text-6xl font-bold tracking-tight text-zinc-500">
                            Configure your
                            <span className="text-[#ffb87b]"> brand new network</span>
                        </h1>

                        <p className="mt-4 text-lg text-zinc-500">
                            Start by giving your router a 67In :
                        </p>

                        <div className="mt-10">
                            <input
                                type="text"
                                placeholder="Enter hostname..."
                                className="w-full rounded-2xl border border-zinc-200 px-6 py-5 text-lg outline-none focus:border-[#ffb87b] focus:ring-4 focus:ring-[#ffb87b]/20"
                            />
                        </div>

                        <button onClick={() => setStep(2)} className="mt-6 rounded-2xl bg-[#ffb87b] px-8 py-4 text-white font-semibold shadow-md hover:scale-[1.02] transition">
                            Configure
                        </button>
                    </div>
                </div>
            )}

        </>
    );
}