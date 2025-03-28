'use client';

import React from 'react';

// TypeScript type for news articles
type NewsArticle = {
  title: string;
  image: string;
  summary: string;
  category: string;
  date: string;
};

const newsArticles: NewsArticle[] = [
  {
    title: 'Nouvelle collection NFT exclusive',
    image: '/images/default/new.jpg',
    summary: "Découvrez la nouvelle collection NFT qui redéfinit l'art numérique dans la musique.",
    category: 'Collections',
    date: '24 Octobre 2024',
  },
  {
    title: 'Artistes en vedette ce mois-ci',
    image: '/images/default/new.jpg',
    summary: "Explorez les artistes montants qui se démarquent dans l'univers NFT musical.",
    category: 'Artistes',
    date: '22 Octobre 2024',
  },
  {
    title: 'Comment fonctionnent les smart contracts ?',
    image: '/images/default/new.jpg',
    summary: "Un guide rapide pour comprendre les bases des smart contracts et leur rôle dans les NFTs.",
    category: 'Éducation',
    date: '20 Octobre 2024',
  },
  // More articles can be added here
];

const Actuality: React.FC = () => {
  return (
    <div className="min-h-screen bg-black text-white p-8">
      <h1 className="text-4xl font-bold text-yellow-500 mb-12 text-center">Actualités</h1>

      {/* News Section */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-8 max-w-6xl mx-auto">
        {newsArticles.map((article, index) => (
          <div key={index} className="bg-gray-800 p-6 rounded-lg shadow-lg hover:shadow-2xl transition-all">
            <img
              src={article.image}
              alt={article.title}
              className="w-full h-98 object-cover rounded-md mb-4"
            />
            <div className="flex justify-between items-center mb-2">
              <span className="text-yellow-500 font-bold">{article.category}</span>
              <span className="text-gray-400 text-sm">{article.date}</span>
            </div>
            <h2 className="text-2xl font-bold text-yellow-500 mb-4">{article.title}</h2>
            <p className="text-gray-400">{article.summary}</p>
            <button className="mt-4 bg-yellow-500 text-black font-bold py-2 px-4 rounded-lg hover:bg-yellow-600 transition-all">
              Lire plus
            </button>
          </div>
        ))}
      </div>
    </div>
  );
};

export default Actuality;
