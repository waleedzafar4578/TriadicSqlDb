import { useState } from 'react'
import './App.css'
function App() {
  const [inputMessage, setInputMessage] = useState('');
  const [outputMessage, setOutputMessage] = useState('');

  const fetchData = async () => {
    try {
      const response = await fetch('http://localhost:8080/process_json', {
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
        <h1>React App Sending and Receiving JSON</h1>
        <label>
          Input Message:
          <input
              type="text"
              value={inputMessage}
              onChange={(e) => setInputMessage(e.target.value)}
          />
        </label>
        <button onClick={handleButtonClick}>Send to Server</button>
        <p>Output Message: {outputMessage}</p>
      </div>
  );
}

export default App;