import { useState, useEffect } from 'react';
import Link from 'next/link';

const NavBar: React.FC = () => {
  const [menuOpen, setMenuOpen] = useState(false);
  const [isMounted, setIsMounted] = useState(false);
  const [isAuthenticated, setIsAuthenticated] = useState(false); // État d'authentification
  const [profileMenuOpen, setProfileMenuOpen] = useState(false); // État du menu profil

  // Utiliser useEffect pour s'assurer que le rendu est côté client
  useEffect(() => {
    setIsMounted(true); // On sait qu'on est côté client
    // TODO: Remplacez cette ligne par votre logique d'authentification réelle
    setIsAuthenticated(true); // Simule l'authentification pour cet exemple
  }, []);

  if (!isMounted) {
    return null; // Evite le rendu du NavBar avant que le composant soit monté côté client
  }

  return (
    <nav className="bg-black p-6 shadow-lg sticky top-0 z-50 w-full">
    <div className="container mx-auto flex justify-between items-center gap-4">
      <div className="text-2xl font-bold text-yellow-500 flex-shrink-0">Smart Music</div>
        <div className="hidden md:flex space-x-6 items-center">
          <Link href="/" className="hover:text-yellow-400 text-white">Accueil</Link>
          <Link href="/liste-artistes" className="hover:text-yellow-400 text-white">Marché des NFTs</Link>
          <Link href="/actualite/" className="hover:text-yellow-400 text-white">Actualités</Link>
          <Link href="/faq/" className="hover:text-yellow-400 text-white">FAQ</Link>

          {!isAuthenticated ? (
            <div className="relative">
              <button
                onClick={() => setProfileMenuOpen(!profileMenuOpen)}
                onMouseEnter={() => setProfileMenuOpen(true)}
                onMouseLeave={() => setProfileMenuOpen(false)}
                className="flex items-center text-yellow-500 focus:outline-none"
              >
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 11c2.21 0 4-1.79 4-4S14.21 3 12 3s-4 1.79-4 4 1.79 4 4 4zm0 2c-4.42 0-8 1.79-8 4v1h16v-1c0-2.21-3.58-4-8-4z" />
                </svg>
              </button>
              
              {profileMenuOpen && (
                <div
                  className="absolute right-0 mt-2 w-48 bg-gray-800 text-white rounded-lg shadow-lg py-2 z-50"
                  onMouseEnter={() => setProfileMenuOpen(true)}
                  onMouseLeave={() => setProfileMenuOpen(false)}
                >
                  <Link href="/profil" className="block px-4 py-2 hover:bg-gray-700">Profil</Link>
                  <button
                    onClick={() => { setIsAuthenticated(false); setProfileMenuOpen(false); }} // Déconnexion
                    className="block w-full text-left px-4 py-2 text-red-500 hover:bg-gray-700 hover:text-red-400"
                  >
                    Déconnexion
                  </button>
                </div>
              )}
            </div>
          ) : (
            <>
              <Link href="/auth/inscription" className="bg-yellow-500 text-black px-4 py-2 rounded-lg hover:bg-yellow-600">
                S'inscrire
              </Link>
              <Link href="/auth/connexion" className="bg-yellow-500 text-black px-4 py-2 rounded-lg hover:bg-yellow-600">
                Se connecter
              </Link>
            </>
          )}

          {/* A revoir  */}
          {/* <Link href="/auth/inscription" className="bg-yellow-500 text-black px-4 py-2 rounded-lg hover:bg-yellow-600">
                S'inscrire
              </Link>
              <Link href="/auth/connexion" className="bg-yellow-500 text-black px-4 py-2 rounded-lg hover:bg-yellow-600">
                Se connecter
          </Link> */}
          {/* Fin */}
        </div>

        {/* Bouton mobile */}
        <div className="md:hidden flex items-center">
          <button
            className="text-yellow-500 focus:outline-none"
            onClick={() => setMenuOpen(!menuOpen)}
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4 6h16M4 12h16m-7 6h7" />
            </svg>
          </button>
        </div>
      </div>

      {/* Mobile menu */}
      {menuOpen && (
        <div className="md:hidden bg-black p-4 space-y-3">
          <Link href="/" className="block text-yellow-500 hover:text-yellow-400">Accueil</Link>
          <Link href="/liste-artistes" className="block text-yellow-500 hover:text-yellow-400">Marché des NFTs</Link>
          <Link href="/actualite/" className="block text-yellow-500 hover:text-yellow-400">Actualités</Link>
          <Link href="/faq/" className="block text-yellow-500 hover:text-yellow-400">FAQ</Link>
          
          {!isAuthenticated ? (
            <>
              <Link href="/profil" className="block text-yellow-500 hover:text-yellow-400">Profil</Link>
              <button
                onClick={() => setIsAuthenticated(false)} // Déconnexion
                className="block w-full text-left text-red-500 hover:bg-gray-700 hover:text-red-400 px-4 py-2 rounded-lg"
              >
                Déconnexion
              </button>
            </>
          ) : (
            <>
              <Link href="/auth/inscription" className="block bg-yellow-500 text-black px-4 py-2 rounded-lg hover:bg-yellow-600">
                S'inscrire
              </Link>
              <Link href="/auth/connexion" className="block bg-yellow-500 text-black px-4 py-2 rounded-lg hover:bg-yellow-600">
                Se connecter
              </Link>
            </>
          )}
        </div>
      )}
    </nav>
  );
};

export default NavBar;
