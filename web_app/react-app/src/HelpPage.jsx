import React, { useState, useEffect } from 'react';

function HelpPage() {
    const [helpData, setHelpData] = useState(null);

    useEffect(() => {
        fetch('http://localhost:8080/help')
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
            <h1>Help Page</h1>
            {helpData && <p>{helpData.message}</p>}
        </div>
    );
}

export default HelpPage;