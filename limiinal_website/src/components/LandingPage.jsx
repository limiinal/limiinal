import React from 'react';
import { ReactTyped } from 'react-typed';
import { useNavigate } from "react-router-dom";

export const LandingPage = () => {
  let navigate = useNavigate(); 
  const routeChange = () =>{ 
    navigate("/about");
  }
  const routeGitHub = () =>{
    const newWindow = window.open("https://github.com/limiinal/limiinal", '_blank', 'noopener,noreferrer');
    if (newWindow) newWindow.opener = null;
  }

  return (
    <div className="bg-gray-900 text-white">
      <section className="min-h-screen flex items-center justify-center px-6 md:px-20">
        <div className="text-center max-w-3xl">
          <h1 className="text-4xl md:text-6xl outfit-bold mb-6">
            <ReactTyped strings={["Limiinal."]} typeSpeed={100}/>
          </h1>
          <p className="text-3xl text-gray mb-1 outfit-mid">
            Chat seamlessly and safely.
            </p>
            <p className="text-xl text-gray mb-8 outfit-light">
            Decentralised. Open-source. Privacy focused.
            </p>
          
          <div className="flex flex-col md:flex-row justify-center gap-4">
            <button className="bg-indigo-600 hover:bg-indigo-700 px-8 py-3 rounded-lg outfit-bold transition" onClick={routeGitHub}>
              GitHub
            </button>
            <button className="bg-slate-500 hover:bg-slate-600 px-8 py-3 rounded-lg outfit-bold transition" onClick={routeChange}>
              About
            </button>
            
          </div>
        </div>
      </section>

      <footer className="py-5 px-6 md:px-20 bg-gray-800 text-center">
        <p className="text-gray-500 outfit-light">
          © 2024 Limiinal. All rights reserved.
        </p>
      </footer>
    </div>
  );
};



