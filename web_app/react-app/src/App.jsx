import { useState } from 'react'
import './App.css'
import Navbar from './navi.jsx';
import ConnectionChecker from "./chaecker.jsx";
import AppRouter from "./Route.jsx";
function App() {
  const [inputMessage, setInputMessage] = useState('');
  const [outputMessage, setOutputMessage] = useState('');

  const fetchData = async () => {
    try {
      const response = await fetch('http://192.168.18.45:8080/process_json', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          message: inputMessage }),
      });

      const data = await response.json();
      setOutputMessage(data.reversed_message);
    } catch (error) {
      console.error('Error fetching data:', error);
    }
  };

  const handleButtonClick = () => {
    fetchData();
  };

  return (
      <div>
        <h1>TRIADIC SQL DATABASE</h1>
        <AppRouter/>
          <ConnectionChecker/>

      </div>
  );
}

export default App;