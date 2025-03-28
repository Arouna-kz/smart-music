'use client';
import { useState } from 'react';
import Head from 'next/head';
import { Swiper, SwiperSlide } from 'swiper/react';
import 'swiper/swiper-bundle.css'; // Import Swiper styles
import { Autoplay, EffectFade, EffectCube, Pagination } from 'swiper/modules'; // Ajout des effets et modules

export default function Home() {

  return (
    <div className="bg-black text-white min-h-screen">
      <Head>
        <title>Chaingenius Technologies | NFT Music Côte d'Ivoire</title>
        <meta name="description" content="Découvrez les NFTs musicaux ivoiriens sur Chaingenius Technologies." />
        <link rel="icon" href="/favicon.ico" />
      </Head>


      {/* Hero Section */}
      <section className="bg-black py-24  flex flex-col items-center justify-center text-center space-y-6">
        <h1 className="text-4xl md:text-6xl font-extrabold text-yellow-500">
          Découvrez les NFT Musicaux de la Côte d'Ivoire
        </h1>
        <p className="text-xl md:text-2xl max-w-3xl">
          Chaingenius Technologies vous offre une plateforme unique pour minter, acheter et vendre des NFTs musicaux créés par des artistes talentueux de la Côte d'Ivoire.
        </p>
        <div className="flex flex-col md:flex-row md:space-x-4 space-y-4 md:space-y-0">
          <a href="#" className="bg-yellow-500 text-black px-8 py-4 rounded-full text-lg font-semibold hover:bg-yellow-600 transition-all">
            Commencez à minter
          </a>
          <a href="#" className="border-2 border-yellow-500 text-yellow-500 px-8 py-4 rounded-full text-lg font-semibold hover:bg-yellow-500 hover:text-black transition-all">
            Explorer les NFTs
          </a>
        </div>
      </section>

      {/* Section Showcase - NFT Populaires */}
      <section id="showcase" className="py-24 bg-gray-900 text-center">
        <h2 className="text-3xl font-bold text-yellow-500 mb-12">NFT Populaires</h2>
         <div className="max-w-7xl mx-auto px-4">
          <Swiper
            spaceBetween={30}
            //slidesPerView={1}  // Nombre de slides affichés par défaut
            autoplay={{ delay: 3000, disableOnInteraction: false }}
            speed={800}
            effect="coverflow" // Les autres effets: coverflow, slide, cube
            modules={[Autoplay, EffectFade, Pagination]}
            pagination={{ clickable: true }}
            breakpoints={{
              640: { slidesPerView: 1 },   // 1 slide pour les écrans < 640px
              768: { slidesPerView: 2 },   // 2 slides pour les écrans >= 768px
              1024: { slidesPerView: 3 },  // 3 slides pour les écrans >= 1024px
            }}
          >
            <SwiperSlide>
              <div className="bg-black border border-yellow-500 p-6 rounded-lg hover:shadow-lg transition-shadow">
                <img src="/images/default/cd1.jpg" alt="NFT 1" className="w-full h-64 object-cover rounded-md" />
                <h3 className="text-xl font-semibold mt-4">DJ Arafat - Clip Exclu</h3>
                <p className="text-gray-400">Une performance incroyable de DJ Arafat dans un clip jamais vu.</p>
              </div>
            </SwiperSlide>
            <SwiperSlide>
              <div className="bg-black border border-yellow-500 p-6 rounded-lg hover:shadow-lg transition-shadow">
                <img src="/images/default/cd1.jpg" alt="NFT 2" className="w-full h-64 object-cover rounded-md" />
                <h3 className="text-xl font-semibold mt-4">Josey - Nouveaux Sons</h3>
                <p className="text-gray-400">Écoutez le dernier titre exclusif de Josey, disponible en NFT.</p>
              </div>
            </SwiperSlide>
            <SwiperSlide>
              <div className="bg-black border border-yellow-500 p-6 rounded-lg hover:shadow-lg transition-shadow">
                <img src="/images/default/cd1.jpg" alt="NFT 3" className="w-full h-64 object-cover rounded-md" />
                <h3 className="text-xl font-semibold mt-4">Kerozen - Clip inédit</h3>
                <p className="text-gray-400">Un clip exclusif de Kerozen avec des beats enflammés.</p>
              </div>
            </SwiperSlide>
            <SwiperSlide>
              <div className="bg-black border border-yellow-500 p-6 rounded-lg hover:shadow-lg transition-shadow">
                <img src="/images/default/cd1.jpg" alt="NFT 1" className="w-full h-64 object-cover rounded-md" />
                <h3 className="text-xl font-semibold mt-4">Didi B - Clip Exclu</h3>
                <p className="text-gray-400">Une performance incroyable de Didi B dans un clip jamais vu.</p>
              </div>
            </SwiperSlide>
      
          </Swiper>
        </div>

      </section>

      {/* Section About */}
      <section id="about" className="py-24 bg-black text-center">
        <h2 className="text-3xl font-bold text-yellow-500 mb-8">À propos de Chaingenius Technologies</h2>
        <p className="text-gray-400 max-w-2xl mx-auto">
          Chaingenius Technologies est une start-up ivoirienne innovante spécialisée dans les NFTs musicaux. Nous offrons aux artistes la possibilité de tokeniser leur musique et de la vendre en tant que NFT, permettant ainsi aux fans de posséder une partie unique de l'histoire musicale de la Côte d'Ivoire.
        </p>
      </section>

      {/* Section Contact */}
      <section id="contact" className="py-24 bg-gray-900 text-center">
        <h2 className="text-3xl font-bold text-yellow-500 mb-8">Contactez-nous</h2>
        <p className="text-gray-400 max-w-xl mx-auto mb-12">Pour toute question ou demande, n'hésitez pas à nous contacter.</p>
        <a href="mailto:info@chaingenius.io" className="bg-yellow-500 text-black px-6 py-4 rounded-lg hover:bg-yellow-600 transition-all">info@chaingenius.io</a>
      </section>

      {/* Footer */}
      
    </div>
  );
}
