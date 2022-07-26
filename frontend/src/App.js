import "./App.css";
import { useEffect, useState } from "react";

const App = () => {
  const [walletAddress, setWalletAddress] = useState(null)
  const checkIfWalletIsConnected = async () => {
    try {
      const { solana } = window;
      if (solana) {
        if (solana.isPhantom) {
          console.log("Phantom wallet found!");
          const response = await solana.connect({ onlyIfTrusted: true });
          console.log(
            "Connect with public key: ",
            response.publicKey.toString()
          );
          setWalletAddress(response.publicKey.toString())
        } else {
          alert("Solana object not found! Get a Phantom wallet");
        }
      }
    } catch (error) {
      console.error(error);
    }
  };

  //function for connecting wallet
  const connectWallet = async () => {

  }

  const renderNotConnectContainer = () => {
    <button onClick={connectWallet}>Connect to wallet</button>
  };

  useEffect(() => {
    //run when page is loaded
    const onLoad = async () => {
      await checkIfWalletIsConnected();
    };
    window.addEventListener("load", onLoad);
    //return eventListener when window close
    return () => window.removeEventListener("load", onLoad);
  }, []);

  return (
  <div className="App">{!walletAddress && renderNotConnectContainer()}</div>
  )
};

export default App;
