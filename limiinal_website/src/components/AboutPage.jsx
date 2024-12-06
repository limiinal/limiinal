import React from 'react';
import { ReactTyped } from 'react-typed';

export const About = () => {
  const routeGitHub = () =>{
    const newWindow = window.open("https://github.com/limiinal/limiinal", '_blank', 'noopener,noreferrer');
    if (newWindow) newWindow.opener = null;
  }
  return (
    <div className="bg-gray-900 text-white">
      <section className="min-h-screen flex items-center justify-center px-6 md:px-20">
        <div className="text-center max-w-3xl">
          <h1 className="text-4xl md:text-6xl outfit-bold mb-6">
            <ReactTyped strings={["Tech Stack"]} typeSpeed={100}/>
          </h1>
          <p className="text-3xl text-gray mb-8 outfit-mid">
            Modern languages and secure communication protocols. Scroll to learn more.
            </p> 

          <div className="flex flex-col md:flex-row justify-center gap-4">
            <button className="bg-indigo-600 hover:bg-indigo-700 px-8 py-3 rounded-lg outfit-bold transition" onClick={routeGitHub}>
              Try now on GitHub
            </button>
          </div>


        </div>
      </section>

      <section className="py-20 px-6 md:px-20 bg-gray-800">
        <h2 className="text-3xl md:text-5xl outfit-bold text-center mb-10">
            <ReactTyped strings={["Why Limiinal?"]} typeSpeed={100}/>
        </h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-10">
          {[
            {
              icon: '🔒',
              title: 'End-to-End Encryption',
              description: 'Your messages are secured and always only accessible to you and the recipient.',
            },
            {
              icon: '🧩',
              title: 'Decentralised Network',
              description: 'We use libp2p for secure peer to peer networking. This means your data is never stored on any external servers.',
            },
            {
              icon: '🦀',
              title: 'Rust Language',
              description: 'We developed Limiinal entirely in Rust. This ensures memory safety and you can be sure memory exploits will not happen. (Touch wood, hey?)',
            },
            
          ].map((feature, index) => (
            <div key={index} className="text-center p-6 bg-gray-700 rounded-lg shadow-lg">
              <div className="text-4xl mb-4">{feature.icon}</div>
              <h3 className="text-xl outfit-mid mb-2">{feature.title}</h3>
              <p className="text-gray-400 outfit-mid">{feature.description}</p>
            </div>
          ))}
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

