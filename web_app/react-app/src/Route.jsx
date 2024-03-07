// Router.js
import React from 'react';
import {BrowserRouter as Router, Route, Link, Routes} from 'react-router-dom';
import EditorPage from './EditorPage';
import ResultPage from './ResultPage';
import HelpPage from './HelpPage';

const AppRouter = () => {
    return (
        <Router>
            <nav>
                <Link to="/editor">Editor</Link>
                <Link to="/result">Result</Link>
                <Link to="/help">Help</Link>
            </nav>
            <Routes>
                <Route path="/editor" element={<EditorPage />} />
                <Route path="/result" element={<ResultPage />} />
                <Route path="/help" element={<HelpPage />} />
            </Routes>
        </Router>
    );
};

export default AppRouter;
