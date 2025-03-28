'use client';

import Link from 'next/link';
import { useRouter } from 'next/router';


export default function ArtistProfile() {
  const router = useRouter();
  const { artistName } = router.query;


  const artist = {
    name: artistName,
    bio: 'Artist biography goes here. The artist is known for their unique style and contributions to the music industry.',
    image: '/images/default/artiste.jpg',
    socialLinks: {
      twitter: 'https://twitter.com/artist',
      instagram: 'https://instagram.com/artist',
    },
    currentWorks: [
      { title: 'NFT 1', mediaType: 'Audio', image: '/images/default/cd2.jpg', price: '0.5 ETH', available: 5 },
      { title: 'NFT 2', mediaType: 'Video', image: '/images/default/cd2.jpg', price: '1.2 ETH', available: 2 },
    ],
    soldWorks: [
      { title: 'NFT 3', mediaType: 'Audio', image: '/images/default/cd2.jpg', soldFor: '2.0 ETH' },
      { title: 'NFT 4', mediaType: 'Video', image: '/images/default/cd2.jpg', soldFor: '1.5 ETH' },
    ],
  };

  return (
    <div className="min-h-screen bg-black text-white p-8">
      {/* Header Section */}
      <div className="flex flex-col md:flex-row items-center md:items-start">
        <img 
          src={artist.image} 
        //   alt={artist.name} 
          className="w-40 h-40 object-cover rounded-full border-4 border-yellow-500 mb-4 md:mb-0 md:mr-8"
        />
        <div className="text-center md:text-left">
          <h1 className="text-4xl font-bold text-yellow-500">{artist.name}</h1>
          <p className="mt-4">{artist.bio}</p>
          <div className="mt-4 flex justify-center md:justify-start space-x-4">
            <Link href={artist.socialLinks.twitter} className="text-yellow-500 hover:text-yellow-600">Twitter</Link>
            <Link href={artist.socialLinks.instagram} className="text-yellow-500 hover:text-yellow-600">Instagram</Link>
          </div>
        </div>
      </div>

    {/* Current Works Section */}
<div className="mt-12">
  <h2 className="text-2xl font-bold text-yellow-500 mb-4">Œuvres en cours</h2> {/* Diminution de la taille du titre */}
  <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4"> {/* Plus de colonnes et moins d'espace */}
    {artist.currentWorks.map((work, index) => (
      <div key={index} className="bg-gray-800 p-4 rounded-lg shadow-md"> {/* Réduction du padding */}
        <img 
          src={work.image} 
          alt={work.title} 
          className="w-full h-90 object-cover rounded-md mb-2" 
        />
        <h3 className="text-lg font-bold text-yellow-500">{work.title}</h3> {/* Taille du texte réduite */}
        <p className="text-gray-400 text-sm">Type: {work.mediaType}</p> {/* Texte plus petit */}
        <p className="text-gray-400 text-sm">Disponible: {work.available} NFT(s)</p>
        <p className="text-md font-bold text-yellow-500 mt-2">{work.price}</p> {/* Police plus petite */}
        <button className="mt-2 bg-yellow-500 text-black font-bold py-1 px-3 rounded-lg hover:bg-yellow-600 transition-all"> {/* Bouton réduit */}
          Acheter
        </button>
      </div>
    ))}
  </div>
</div>

{/* Sold Works Section */}
<div className="mt-8">
  <h2 className="text-2xl font-bold text-yellow-500 mb-4">Œuvres vendues</h2> {/* Taille du titre ajustée */}
  <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4"> {/* Diminution de l'écart */}
    {artist.soldWorks.map((work, index) => (
      <div key={index} className="bg-gray-800 p-4 rounded-lg shadow-md"> {/* Réduction du padding */}
        <img 
          src={work.image} 
          alt={work.title} 
          className="w-full h-90 object-cover rounded-md mb-2" 
        />
        <h3 className="text-lg font-bold text-yellow-500">{work.title}</h3> {/* Texte réduit */}
        <p className="text-gray-400 text-sm">Type: {work.mediaType}</p> {/* Réduction de la taille du texte */}
        <p className="text-md font-bold text-yellow-500 mt-2">Vendu pour {work.soldFor}</p> {/* Police ajustée */}
      </div>
    ))}
  </div>
</div>

    </div>
  );
}
