'use client';
import { useState, ChangeEvent, FormEvent } from 'react';
import Head from 'next/head';

export default function Login() {
  const [email, setEmail] = useState<string>('');  // Spécification du type string
  const [password, setPassword] = useState<string>('');  // Spécification du type string

  // Typage de l'événement e en tant que FormEvent
  const handleLogin = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    // Logique de connexion ici
    console.log("Email:", email);
    console.log("Password:", password);
  };

  // Typage de l'événement onChange en tant que ChangeEvent
  const handleEmailChange = (e: ChangeEvent<HTMLInputElement>) => {
    setEmail(e.target.value);
  };

  const handlePasswordChange = (e: ChangeEvent<HTMLInputElement>) => {
    setPassword(e.target.value);
  };

  return (
    <div className="bg-black text-white min-h-screen flex items-center justify-center">
      <Head>
        <title>Connexion | Chaingenius Technologies</title>
        <meta name="description" content="Connectez-vous à Chaingenius Technologies, la plateforme de NFT musicaux." />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <div className="bg-gray-900 p-8 rounded-lg shadow-lg max-w-md w-full">
        <h1 className="text-3xl font-bold text-yellow-500 text-center mb-6">Connexion</h1>
        <form onSubmit={handleLogin} className="space-y-6">
          <div>
            <label className="block mb-2 text-yellow-500">Adresse e-mail</label>
            <input
              type="email"
              value={email}
              onChange={handleEmailChange}  // Typage correct de l'événement
              className="w-full p-4 bg-black border border-yellow-500 rounded-lg focus:outline-none focus:ring-2 focus:ring-yellow-500"
              placeholder="Entrez votre e-mail"
              required
            />
          </div>
          <div>
            <label className="block mb-2 text-yellow-500">Mot de passe</label>
            <input
              type="password"
              value={password}
              onChange={handlePasswordChange}  // Typage correct de l'événement
              className="w-full p-4 bg-black border border-yellow-500 rounded-lg focus:outline-none focus:ring-2 focus:ring-yellow-500"
              placeholder="Entrez votre mot de passe"
              required
            />
          </div>
          <div className="text-right">
            <a href="#" className="text-yellow-500 hover:text-yellow-400">Mot de passe oublié ?</a>
          </div>
          <button type="submit" className="w-full bg-yellow-500 text-black p-4 rounded-lg font-bold hover:bg-yellow-600 transition-all">
            Connexion
          </button>
          <p className="text-center text-gray-400 mt-4">Ou connectez-vous avec :</p>
          <div className="flex space-x-4 justify-center mt-2">
            <button className="bg-black border border-yellow-500 text-yellow-500 px-4 py-2 rounded-lg hover:bg-yellow-500 hover:text-black transition-all">
              MetaMask
            </button>
            <button className="bg-black border border-yellow-500 text-yellow-500 px-4 py-2 rounded-lg hover:bg-yellow-500 hover:text-black transition-all">
              Magic Link
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
