'use client';

import { useState } from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faChevronDown, faChevronUp } from '@fortawesome/free-solid-svg-icons';

export default function FAQ() {
  const [activeIndex, setActiveIndex] = useState<number | null>(null);

  // Example FAQ data
  const faqs = [
    {
      question: 'Comment créer un NFT sur la plateforme ?',
      answer: 'Pour créer un NFT, vous devez vous inscrire, accéder à la section de création d’artistes et suivre les instructions pour téléverser vos fichiers et définir les détails de votre NFT.',
    },
    {
      question: 'Comment acheter un NFT sur la plateforme ?',
      answer: 'Pour acheter un NFT, parcourez le marketplace, sélectionnez le NFT souhaité et suivez les instructions pour finaliser la transaction.',
    },
    {
      question: 'Quels types de médias sont acceptés pour créer des NFT ?',
      answer: 'Nous acceptons plusieurs types de médias, notamment des images, vidéos, et fichiers audio pour la création de NFT.',
    },
    {
      question: 'Qui contacter en cas de problème technique ?',
      answer: 'Pour toute assistance technique, veuillez contacter notre support via l’email support@votreplateforme.com ou via notre chat en ligne.',
    },
  ];

  const toggleFAQ = (index: number) => {
    setActiveIndex(activeIndex === index ? null : index);
  };

  return (
    <div className="min-h-screen bg-black text-white p-8">
      <h1 className="text-4xl font-bold text-yellow-500 mb-12 text-center">FAQ</h1>

      {/* FAQ Section */}
      <div className="max-w-4xl mx-auto">
        {faqs.map((faq, index: number) => (
          <div key={index} className="mb-6">
            <div
              className="bg-gray-800 p-4 rounded-lg shadow-md cursor-pointer hover:bg-gray-700 transition-colors flex justify-between items-center"
              onClick={() => toggleFAQ(index)}
            >
              <h2 className="text-xl font-bold text-yellow-500">
                {faq.question}
              </h2>
              {/* Icon */}
              <FontAwesomeIcon
                icon={activeIndex === index ? faChevronUp : faChevronDown}
                className="text-yellow-500"
              />
            </div>
            {activeIndex === index && (
              <div className="mt-2 bg-gray-900 p-4 rounded-lg text-gray-400">
                <p>{faq.answer}</p>
              </div>
            )}
          </div>
        ))}
      </div>

      {/* Support Section */}
      <div className="mt-16 text-center">
        <h2 className="text-3xl font-bold text-yellow-500 mb-4">Besoin d'assistance ?</h2>
        <p className="text-gray-400 mb-4">Pour toute question supplémentaire ou problème technique, contactez notre service client.</p>
        <button className="bg-yellow-500 text-black font-bold py-2 px-6 rounded-lg hover:bg-yellow-600 transition-all">
          Contacter le Support
        </button>
      </div>
    </div>
  );
}
