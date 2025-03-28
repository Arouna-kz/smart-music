import { useState, useEffect } from 'react';
import Link from 'next/link';

const FooterOne: React.FC = () => {
  const [isMounted, setIsMounted] = useState(false);

  // Utiliser useEffect pour s'assurer que le rendu est côté client
  useEffect(() => {
    setIsMounted(true); // Une fois que le composant est monté, on sait qu'on est côté client
  }, []);

  if (!isMounted) {
    return null; // Evite le rendu du NavBar avant que le composant soit monté côté client
  }

  return (
    <footer className="bg-black py-6 text-center text-gray-500">
        © 2024 Chaingenius Technologies. Tous droits réservés.
        <div className="mt-2">
          <span className="cursor-pointer hover:text-yellow-500">Français</span> | <span className="cursor-pointer hover:text-yellow-500">English</span>
        </div>
    </footer>
  );
};

export default FooterOne;
