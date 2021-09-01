import * as firebase from "firebase";

export const loadFirebase = () => {
  const isProd = process.env.NODE_ENV === "production";

  // Initialisation de Firebase (par défaut on utilise la configuration de production)
  let firebaseConfig = {
    apiKey: "AIzaSyCcSzoHDId-mbPMTAplfFDD0NIferf5GF8",
    authDomain: "piteo-prod.firebaseapp.com",
    databaseURL: "https://piteo-prod.firebaseio.com",
    projectId: "piteo-prod",
    storageBucket: "piteo-prod.appspot.com",
    messagingSenderId: "36889636677",
    appId: "1:36889636677:web:5a109aedc232bc5733582c",
    measurementId: "G-FFW38XX61K",
  };

  // En dev on bascule sur un environnement sandbox
  if (!isProd) {
    firebaseConfig = {
      apiKey: "AIzaSyCtM_89aBwv2PRZV_lAx3YrhXebKaFEJE0",
      authDomain: "piteo-dev.firebaseapp.com",
      databaseURL: "",
      projectId: "piteo-dev",
      storageBucket: "piteo-dev.appspot.com",
      messagingSenderId: "75820091109",
      appId: "1:75820091109:web:592fbb7db1c45ea1e1b60c",
      measurementId: "",
    };
  }

  // Initialize Firebase
  firebase.initializeApp(firebaseConfig);
  firebase.analytics();
};
