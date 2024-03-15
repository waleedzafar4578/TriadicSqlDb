// eslint-disable-next-line no-unused-vars
import React, { useState, useEffect } from 'react';

function ResultPage() {
    const [helpData, setHelpData] = useState(null);

    useEffect(() => {
        fetch('http://localhost:8080/result')
            .then(response => response.json())
            .then(data => {
                setHelpData(data);
            })
            .catch(error => {
                console.error('Error fetching help data:', error);
            });
    }, []);

    return (
        <div>
            <h1>Result</h1>
            {helpData && <p>{helpData.message}</p>}
        </div>
    );
}

export default ResultPage;