import { useState } from 'react';
import Link from 'next/link';
import Image from "next/image";


export default function ArtistList() {
  const [searchTerm, setSearchTerm] = useState('');

  const artists = [
    { name: 'Artist 1', description: 'Description...', image: '/images/default/artiste.jpg' },
    { name: 'Artist 2', description: 'Description...', image: '/images/default/artiste.jpg' },
    { name: 'Artist 3', description: 'Description...', image: '/images/default/artiste.jpg' },
    { name: 'Artist 4', description: 'Description...', image: '/images/default/artiste.jpg' },
    { name: 'Artist 5', description: 'Description...', image: '/images/default/artiste.jpg' },
    { name: 'Artist 6', description: 'Description...', image: '/images/default/artiste.jpg' },
    // Add more artists here
  ];

  // Filter artists based on the search term
  const filteredArtists = artists.filter(artist =>
    artist.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  return (
    <div className="min-h-screen bg-black text-white p-8">
      <h1 className="text-4xl font-bold text-center text-yellow-500 mb-8">Artistes</h1>

      {/* Search bar */}
      <div className="mb-8 text-center">
        <input
          type="text"
          placeholder="Rechercher un artiste..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="bg-gray-700 text-white px-4 py-2 rounded-lg focus:outline-none focus:ring-2 focus:ring-yellow-500"
        />
      </div>

      {/* Artist grid */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-8">
        {filteredArtists.length > 0 ? (
          filteredArtists.map((artist, index) => (
            <div key={index} className="bg-gray-800 p-6 rounded-lg shadow-lg text-center">
              <Image src={artist.image} alt={artist.name} width={300} height={256} className="w-full h-98 object-cover rounded-md" />
              
              <h2 className="text-2xl font-bold text-yellow-500 mt-4">{artist.name}</h2>
              <p className="text-gray-400 mt-2">{artist.description}</p>

              {/* Links for Profile and Marketplace */}
              <div className="mt-4 flex justify-around">
                {/* Profile Button */}
                <Link legacyBehavior href={`/liste-artistes/artiste-profil/${artist.name.toLowerCase().replace(/\s+/g, '')}`}>
                  {/* <a className="bg-gray-500 text-black font-bold py-2 px-6 rounded-lg hover:bg-gray-600 transition-all"> */}
                  <a className='border-2 border-yellow-500 text-yellow-500 px-6 py-2 rounded-lg text-lg font-semibold hover:bg-yellow-500 hover:text-black transition-all'>
                    Profil
                  </a>
                </Link>

                {/* Marketplace Button */}
                <Link legacyBehavior href={`/marche-nft/${artist.name.toLowerCase().replace(/\s+/g, '')}`}>
                  <a className="bg-yellow-500 text-black font-bold py-2 px-6 rounded-lg hover:bg-yellow-600 transition-all">
                    Marketplace
                  </a>
                </Link>
              </div>
            </div>
          ))
        ) : (
          <p className="text-gray-400 text-center col-span-full">Aucun artiste trouvé</p>
        )}
      </div>
    </div>
  );
}
