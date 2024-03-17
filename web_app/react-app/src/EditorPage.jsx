import  { useState } from 'react';
import MyEditor from "./editor.jsx";


const EditorPage = () => {
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
            <h3>TextEditor</h3>
            <button onClick={handleButtonClick}>Run</button>
            <button onClick={handleButtonClick}>Run All</button>
            <MyEditor
                value={inputMessage}
                onChange={(value) => setInputMessage(value)}
            />

            <p>Result:</p>
            <h4>{outputMessage}</h4>
        </div>
    );
};

export default EditorPage;
