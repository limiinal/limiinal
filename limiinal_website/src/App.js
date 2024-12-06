import './App.css';
import { LandingPage } from "./components/LandingPage.jsx";
import { About } from "./components/AboutPage.jsx";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";

function App() {
  return (
    <div className='App'>
      <Router>
        <Routes>
          <Route path = "/" element = {<LandingPage />} />
          <Route path = "/about" element = {<About />} />
        </Routes>
      </Router>
    </div>
    
  );
}

export default App;
