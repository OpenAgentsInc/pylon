import "./App.css"
import "./styles/global.scss"
import Card from "./components/Card"

function App() {
  return (
    <main className="container">
      <Card title="Welcome to Pylon">
        <p>Awaiting connection</p>
      </Card>
    </main>
  );
}

export default App;
