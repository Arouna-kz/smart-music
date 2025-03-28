import { useRouter } from 'next/router';
import { useState } from 'react';

export default function ArtistMarketplace() {
  const router = useRouter();
  const { artistName } = router.query;

  const [searchTerm, setSearchTerm] = useState('');
  const [genre, setGenre] = useState('');
  const [mediaType, setMediaType] = useState('');
  const [maxPrice, setMaxPrice] = useState('');

  const nfts = [
    { title: 'NFT 1', genre: 'Hip-Hop', mediaType: 'Audio', price: '0.5 ETH', available: 10, image: '/nft1.jpg' },
    { title: 'NFT 2', genre: 'Pop', mediaType: 'Video', price: '1.2 ETH', available: 0, image: '/nft2.jpg' }, // Indisponible pour l'exemple
    { title: 'NFT 3', genre: 'Rock', mediaType: 'Audio', price: '0.8 ETH',available: 100, image: '/nft3.jpg' },
  ];

  // Filtrer les NFTs en fonction des critères de recherche
  const filteredNFTs = nfts.filter(nft =>
    nft.title.toLowerCase().includes(searchTerm.toLowerCase()) &&
    (!genre || nft.genre === genre) &&
    (!mediaType || nft.mediaType === mediaType) &&
    (!maxPrice || parseFloat(nft.price) <= parseFloat(maxPrice))
  );

  return (
    <div className="min-h-screen bg-black text-white p-8">
      <h1 className="text-4xl font-bold text-yellow-500 mb-8 text-center">{artistName}'s Marketplace</h1>

      {/* Filtres */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
        <input
          type="text"
          placeholder="Rechercher un NFT..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="bg-gray-700 text-white px-4 py-2 rounded-lg focus:outline-none focus:ring-2 focus:ring-yellow-500"
        />
        <select
          value={genre}
          onChange={(e) => setGenre(e.target.value)}
          className="bg-gray-700 text-white px-4 py-2 rounded-lg focus:outline-none focus:ring-2 focus:ring-yellow-500"
        >
          <option value="">Sélectionner le genre</option>
          <option value="Hip-Hop">Hip-Hop</option>
          <option value="Pop">Pop</option>
        </select>
        <select
          value={mediaType}
          onChange={(e) => setMediaType(e.target.value)}
          className="bg-gray-700 text-white px-4 py-2 rounded-lg focus:outline-none focus:ring-2 focus:ring-yellow-500"
        >
          <option value="">Type de média</option>
          <option value="Audio">Audio</option>
          <option value="Video">Vidéo</option>
        </select>
        <input
          type="number"
          placeholder="Prix max (ETH)"
          value={maxPrice}
          onChange={(e) => setMaxPrice(e.target.value)}
          className="bg-gray-700 text-white px-4 py-2 rounded-lg focus:outline-none focus:ring-2 focus:ring-yellow-500"
        />
      </div>

      {/* Grille des NFTs */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-8">
        {filteredNFTs.map((nft, index) => (
          <div key={index} className="relative bg-gray-800 p-4 rounded-lg shadow-md flex flex-col h-full">
            {/* Étiquette triangulaire */}
            <div 
              className={`absolute top-0 right-0 w-0 h-0 border-t-[40px] border-l-[40px] 
                ${nft.available > 0 ? 'border-green-500' : 'border-red-500'} 
                border-t-transparent border-l-transparent`}
            >
              {/* Texte sur l'étiquette */}
              <div 
                className={`absolute top-[-25px] right-[-25px] rotate-45 
                  ${nft.available > 0 ? 'bg-green-500' : 'bg-red-500'} 
                  p-1 rounded-md`}
              >
                {nft.available > 0 ? (
                <p className="text-xs font-bold text-white">Disponible: {nft.available} NFT(s)</p>
                ):(
                  <p className="text-xs font-bold text-white">Indisponible: {nft.available} NFT</p>
                )}
              </div>
            </div>

            {/* Image du NFT */}
            <img 
              src={nft.image} 
              alt={nft.title} 
              className="w-full h-36 object-cover rounded-md mb-4" 
            />

            {/* Titre du NFT */}
            <h2 className="text-lg font-bold text-yellow-500 text-center mb-3">{nft.title}</h2>

            {/* Détails : Genre et Type */}
            <div className="flex justify-between text-gray-400 text-sm mb-3">
              <span>Genre: {nft.genre}</span>
              <span>Type: {nft.mediaType}</span>
            </div>

            {/* Prix et bouton d'achat */}
            <div className="flex justify-between items-center mt-auto">
              <p className="text-lg font-bold text-yellow-500">{nft.price}</p>
              <button className="bg-yellow-500 text-black font-bold py-2 px-4 rounded-lg hover:bg-yellow-600 transition-all">
                Acheter
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
