import React from 'react';
import './App.css';
import eliteDlogo from './assets/eliteDlogo.png';

function App() {
  return (
    <div className="app-container">
      <div className="pathfinder">
        <div className="header">
          <img src={eliteDlogo} alt="Elite Dangerous Logo" className="logo" />
          <h1>ELITE DANGEROUS PATHFINDER</h1>
        </div>

        <div className="content">
          <div className="neutron-router">
            <h2>Neutron Router</h2>
            <p>Travel between neutron star systems</p>

            <div className="input-group">
              <label>Source System</label>
              <input type="text" placeholder="Source System" />
            </div>

            <button className="btn">Add via +</button>
            <button className="btn">Reverse ↔</button>

            <div className="input-group">
              <label>Destination System</label>
              <input type="text" placeholder="Destination System" />
            </div>

            <div className="input-double">
              <input type="text" placeholder="Range(LY)" />
              <input type="text" placeholder="Efficiency(%)" />
            </div>

            <button className="calculate-btn">Calculate</button>
          </div>

          <div className="result-section">
            <h2>Result</h2>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;


