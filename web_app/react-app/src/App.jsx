import { useState } from 'react'
import './App.css'
import Navbar from './navi.jsx';
import ConnectionChecker from "./chaecker.jsx";
import AppRouter from "./Route.jsx";
function App() {

  return (
      <div>
        <h1>TRIADIC SQL DATABASE</h1>
        <AppRouter/>
          <ConnectionChecker/>

      </div>
  );
}

export default App;