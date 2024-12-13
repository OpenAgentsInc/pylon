import "./App.css"
import "./styles/global.scss"
import Card from "./components/Card"
import ClientList from "./components/ClientList"

function App() {
  return (
    <main className="container">
      <Card title="Pylon Node">
        <ClientList />
      </Card>
    </main>
  );
}

export default App;