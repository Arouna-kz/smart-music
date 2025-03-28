This is a [Next.js](https://nextjs.org/) project bootstrapped with [`create-next-app`](https://github.com/vercel/next.js/tree/canary/packages/create-next-app).

## Getting Started

First, run the development server:

```bash
npm run dev
# or
yarn dev
# or
pnpm dev
# or
bun dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying `pages/index.tsx`. The page auto-updates as you edit the file.

[API routes](https://nextjs.org/docs/api-routes/introduction) can be accessed on [http://localhost:3000/api/hello](http://localhost:3000/api/hello). This endpoint can be edited in `pages/api/hello.ts`.

The `pages/api` directory is mapped to `/api/*`. Files in this directory are treated as [API routes](https://nextjs.org/docs/api-routes/introduction) instead of React pages.

This project uses [`next/font`](https://nextjs.org/docs/basic-features/font-optimization) to automatically optimize and load Inter, a custom Google Font.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js/) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/deployment) for more details.


### *****************************************PUSHER SUR GITHUB******************************************************************

Voici les étapes pour pousser ton projet Next.js sur GitHub :

🔹 1. Initialiser Git dans ton projet
Va dans le dossier de ton projet et exécute ces commandes :
cd mon-projet-nextjs  # Remplace par le nom de ton dossier
COMMANDE: git init  # Initialise Git dans le projet

🔹 2. Ajouter les fichiers au suivi Git
Ajoute tous les fichiers à Git :
COMMANDE: git add .

Puis fais un commit :
COMMANDE: git commit -m "Premier commit - Initialisation du projet Next.js"

🔹 3. Créer un dépôt GitHub
Va sur GitHub et connecte-toi.

Clique sur New Repository.
Donne un nom à ton repo (ex: mon-projet-nextjs).
Clique sur Create repository.

🔹 4. Lier ton projet au dépôt GitHub
Copie l’URL fournie par GitHub (ex: https://github.com/ton-utilisateur/mon-projet-nextjs.git) et exécute la commande suivante :
COMMANDE: git remote add origin https://github.com/Arouna-kz/smart-music.git

Vérifie si le remote est bien ajouté :
COMMANDE: git remote -v

🔹 5. Pousser le projet sur GitHub
Enfin, envoie ton projet sur GitHub avec :
COMMANDE: git branch -M main
COMMANDE: git push -u origin main





## ************************Heberger sur cercel***********************************************************************************

Voici les étapes pour déployer ton projet Next.js sur Vercel :

1. Installer Vercel CLI (Facultatif mais utile)
Si tu veux utiliser la ligne de commande, installe Vercel CLI sur ton ordinateur :
COMMANDE : npm install -g vercel

2. Créer un compte Vercel
Va sur https://vercel.com/
Inscris-toi ou connecte-toi avec GitHub, GitLab ou Bitbucket.

3. Connecter ton projet à GitHub (Recommandé)
Pousse ton projet Next.js sur GitHub.
Sur Vercel, clique sur "New Project".
Sélectionne ton dépôt GitHub.
Configure les paramètres (généralement, la détection est automatique).
Clique sur "Deploy".

4. Déployer avec Vercel CLI (Alternative sans GitHub)
Si tu veux déployer directement depuis ton terminal :
Va dans ton dossier de projet :
cd mon-projet-nextjs
Lance la commande suivante :
COMMANDE : vercel

Suis les instructions à l'écran :
Associe ton projet à ton compte Vercel.
Sélectionne les paramètres recommandés.
Attends que le déploiement se termine.

5. Vérifier le déploiement
Une fois terminé, Vercel te fournira une URL du type https://mon-projet.vercel.app/.
Tu peux aussi voir ton projet dans le tableau de bord Vercel.

6. Gérer les mises à jour
Si ton projet est connecté à GitHub, chaque git push sur main ou master déclenchera un déploiement automatique.

Sinon, utilise :
COMMANDE :  vercel --prod