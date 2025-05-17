'use client';
import { useState, ChangeEvent, FormEvent } from 'react';
import Head from 'next/head';

export default function Register() {
  const [username, setUsername] = useState<string>('');  // Nom d'utilisateur
  const [email, setEmail] = useState<string>('');  // E-mail
  const [password, setPassword] = useState<string>('');  // Mot de passe
  const [confirmPassword, setConfirmPassword] = useState<string>('');  // Confirmation mot de passe
  const [accountType, setAccountType] = useState<string>('user');  // Type de compte (utilisateur ou artiste)

  // Gestion de l'inscription
  const handleRegister = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    
    // Vérification que le mot de passe et la confirmation correspondent
    if (password !== confirmPassword) {
      alert("Les mots de passe ne correspondent pas.");
      return;
    }

    // Logique d'inscription ici
    console.log("Username:", username);
    console.log("Email:", email);
    console.log("Password:", password);
    console.log("Account Type:", accountType);
  };

  // Gestion des champs de formulaire
  const handleUsernameChange = (e: ChangeEvent<HTMLInputElement>) => setUsername(e.target.value);
  const handleEmailChange = (e: ChangeEvent<HTMLInputElement>) => setEmail(e.target.value);
  const handlePasswordChange = (e: ChangeEvent<HTMLInputElement>) => setPassword(e.target.value);
  const handleConfirmPasswordChange = (e: ChangeEvent<HTMLInputElement>) => setConfirmPassword(e.target.value);
  const handleAccountTypeChange = (e: ChangeEvent<HTMLSelectElement>) => setAccountType(e.target.value);

  return (
    <div className="bg-black text-white min-h-screen flex items-center justify-center">
      <Head>
        <title>Inscription | Chaingenius Technologies</title>
        <meta name="description" content="Créez un compte sur Chaingenius Technologies, la plateforme de NFT musicaux." />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <div className="bg-gray-900 p-8 my-5 rounded-lg shadow-lg max-w-md w-full">
        <h1 className="text-3xl font-bold text-yellow-500 text-center mb-6">Inscription</h1>
        <form onSubmit={handleRegister} className="space-y-6">
          <div>
            <label className="block mb-2 text-yellow-500">Nom d'utilisateur</label>
            <input
              type="text"
              value={username}
              onChange={handleUsernameChange}
              className="w-full p-4 bg-black border border-yellow-500 rounded-lg focus:outline-none focus:ring-2 focus:ring-yellow-500"
              placeholder="Entrez votre nom d'utilisateur"
              required
            />
          </div>
          <div>
            <label className="block mb-2 text-yellow-500">Adresse e-mail</label>
            <input
              type="email"
              value={email}
              onChange={handleEmailChange}
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
              onChange={handlePasswordChange}
              className="w-full p-4 bg-black border border-yellow-500 rounded-lg focus:outline-none focus:ring-2 focus:ring-yellow-500"
              placeholder="Entrez votre mot de passe"
              required
            />
          </div>
          <div>
            <label className="block mb-2 text-yellow-500">Confirmer le mot de passe</label>
            <input
              type="password"
              value={confirmPassword}
              onChange={handleConfirmPasswordChange}
              className="w-full p-4 bg-black border border-yellow-500 rounded-lg focus:outline-none focus:ring-2 focus:ring-yellow-500"
              placeholder="Confirmez votre mot de passe"
              required
            />
          </div>
          <div>
            <label className="block mb-2 text-yellow-500">Type de compte</label>
            <select
              value={accountType}
              onChange={handleAccountTypeChange}
              className="w-full p-4 bg-black border border-yellow-500 rounded-lg focus:outline-none focus:ring-2 focus:ring-yellow-500"
              required
            >
              <option value="user">Utilisateur</option>
              <option value="artist">Artiste</option>
            </select>
          </div>
          <button type="submit" className="w-full bg-yellow-500 text-black p-4 rounded-lg font-bold hover:bg-yellow-600 transition-all">
            Inscription
          </button>
          <p className="text-center text-gray-400 mt-4">Ou inscrivez-vous avec :</p>
          <div className="flex space-x-4 justify-center mt-2">
            <button className="bg-black border border-yellow-500 text-yellow-500 px-4 py-2 rounded-lg hover:bg-yellow-500 hover:text-black transition-all">
              Phantom
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
