import "./App.css";
import {useEffect} from "react"

const App = () => {
  const checkIfWalletIsConnected = async () => {
    try {
      const { solana } = window;
      if (solana) {
        if (solana.isPhantom) {
          console.log("Phantom wallet found!");
        } else {
          alert("Solana object not found! Get a Phantom wallet");
        }
      }
    } catch (error) {
      console.error(error);
    }
  };
  useEffect(() => {
    //run when page is loaded
    const onLoad = async() => {
      await checkIfWalletIsConnected()
    
    }
    window.addEventListener('load',onLoad)
    //return eventListener when window close
    return () => window.removeEventListener("load", onLoad)
  }, [])
};

export default App;
