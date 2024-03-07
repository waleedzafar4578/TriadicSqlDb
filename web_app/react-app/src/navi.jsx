import React from 'react';

const Navbar = () => {
    const tabs = [
        { text: 'Editor', link: '/editor' },
        { text: 'Result', link: '/result' },
        { text: 'Help', link: '/help' }
    ];

    return (
        <nav>
            {tabs.map(tab => (
                <a key={tab.text} href={tab.link}>{tab.text}</a>
            ))}
        </nav>
    );
};

export default Navbar;