import { useState } from "react";
import "./App.css";
import eliteDlogo from "./assets/eliteDlogo.png";
import { FaTrashAlt } from "react-icons/fa";

function App() {
  const [source, setSource] = useState("");
  const [destination, setDestination] = useState("");
  const [viaSystems, setViaSystems] = useState([]);
  const [range, setRange] = useState("");
  const [weight, setWeight] = useState("");

  const [resultData, setResultData] = useState(null);
  const [loading, setLoading] = useState(false);
  const [selectedAlgorithm, setSelectedAlgorithm] = useState("Dijkstra");

  const handleAddVia = () => {
    setViaSystems([...viaSystems, ""]);
  };

  const handleReverse = () => {
    const temp = source;
    setSource(destination);
    setDestination(temp);
  };

  const handleViaChange = (index, value) => {
    const newViaSystems = [...viaSystems];
    newViaSystems[index] = value;
    setViaSystems(newViaSystems);
  };

  const handleDeleteVia = (index) => {
    const newViaSystems = viaSystems.filter((_, i) => i !== index);
    setViaSystems(newViaSystems);
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);
    setResultData(null);

    const formData = {
      source: parseInt(source),
      destination: parseInt(destination),
      via_systems: viaSystems,
      jump_distance: parseFloat(range),
      weight: parseFloat(weight),
      algorithm: selectedAlgorithm,
    };

    try {
      const response = await fetch("http://localhost:3000/api/path-details", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(formData),
      });

      if (response.ok) {
        const result = await response.json();
        console.log("Form submitted successfully:", result);
        setResultData(result);
      } else {
        console.error("Error submitting form");
      }
    } catch (error) {
      console.error("Error:", error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="app-container">
      <div className="pathfinder">
        <div className="header">
          <img
            src={eliteDlogo}
            alt="Elite Dangerous Logo"
            className="logo"
            onClick={() => window.location.reload()}
          />
          <h1>ELITE DANGEROUS PATHFINDER</h1>
        </div>

        <div className="content">
          <div className="neutron-router">
            <div className="heading-container">
              <h2>Star System Router</h2>
            </div>
            <p>Travel between star systems</p>

            <form onSubmit={handleSubmit}>
              <div className="input-group">
                <label>Source System</label>
                <input
                  type="text"
                  placeholder="Source System"
                  value={source}
                  onChange={(e) => setSource(e.target.value)}
                  required
                />
              </div>

              {viaSystems.map((via, index) => (
                <div key={index} className="input-group via-group">
                  <input
                    type="text"
                    placeholder={`Via System ${index + 1}`}
                    value={via}
                    onChange={(e) => handleViaChange(index, e.target.value)}
                  />
                  <button
                    type="button"
                    className="delete-btn"
                    onClick={() => handleDeleteVia(index)}
                  >
                    <FaTrashAlt />
                  </button>
                </div>
              ))}

              <button
                type="button"
                className="btn add-via-btn"
                onClick={handleAddVia}
              >
                Add Via +
              </button>
              <button
                type="button"
                className="btn reverse-btn"
                onClick={handleReverse}
              >
                Reverse ⇆
              </button>
              <br />
              <br />

              <div className="input-group">
                <label>Destination System</label>
                <input
                  type="text"
                  placeholder="Destination System"
                  value={destination}
                  onChange={(e) => setDestination(e.target.value)}
                  required
                />
              </div>

              <div className="input-double">
                <input
                  type="text"
                  placeholder="Range (LY)"
                  value={range}
                  onChange={(e) => setRange(e.target.value)}
                />
                <input
                  type="text"
                  placeholder="Ship Weight (Tons)"
                  value={weight}
                  onChange={(e) => setWeight(e.target.value)}
                />
              </div>

              <button type="submit" className="calculate-btn">
                Calculate
              </button>
            </form>
          </div>

          <div className="result-section">
            <div className="heading-container">
              <h2>Result</h2>
            </div>

            <div className="algorithm-dropdown">
              <label htmlFor="algorithm">Select Algorithm: </label>
              <select
                id="algorithm"
                value={selectedAlgorithm}
                onChange={(e) => setSelectedAlgorithm(e.target.value)}
              >
                <option value="Dijkstra">Dijkstra</option>
                <option value="A-Star">A-Star</option>
              </select>
            </div>

            {loading && (
              <div className="loader-container">
                <div className="loader"></div>
              </div>
            )}

            {!loading && resultData && (
              <div className="result-details">
                <p>Estimated Jumps: {resultData.jumps}</p>
                <p>Distance: {resultData.distance.toFixed(4)} LY</p>

                <div className="path-container">
                  <table className="result-table">
                    <thead>
                      <tr>
                        <th>S.No</th>
                        <th>System ID</th>
                        <th>Distance (LY)</th>
                        <th>Jumps</th>
                      </tr>
                    </thead>
                    <tbody>
                      {resultData.path.map((system, index) => (
                        <tr
                          key={index}
                          className={`${
                            index === 0
                              ? "source-row"
                              : index === resultData.path.length - 1
                                ? "destination-row"
                                : "intermediate-row"
                          }`}
                        >
                          <td>{index + 1}</td>
                          <td>{resultData.path[index][0]}</td>
                          <td>{resultData.path[index][1].toFixed(4)}</td>
                          <td>{resultData.path[index][2]}</td>
                        </tr>
                      ))}
                      <tr className="total-row">
                        <td></td>
                        <td>Total</td>
                        <td>{resultData.distance.toFixed(4)}</td>
                        <td>{resultData.jumps}</td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;