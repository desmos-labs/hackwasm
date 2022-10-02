import React, {useState} from 'react';
import './App.css';
import {PostList, SearchBar, Title} from "./components";
import Background from './assets/background.png';


function App() {
  const [search, setSearch] = useState<string | undefined>(undefined)

  return (
    <div className="App" style={{ background: `url(${Background})`}}>
      <Title text={"INTERCHAIN INDEXER"} style={{ marginTop: '8%'}}/>
      <SearchBar
        onSubmit={(value) => {
          console.log("submit", value);
          setSearch(value);
        }}
      />
      <PostList searchText={search}/>
    </div>
  );
}

export default App;
