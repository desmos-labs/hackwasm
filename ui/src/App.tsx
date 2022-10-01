import React, {useState} from 'react';
import './App.css';
import {PostList, SearchBar, Title} from "./components";


function App() {
  const [search, setSearch] = useState<string | undefined>(undefined)

  return (
    <div className="App">
      <Title text={"Interchain Browser"}/>
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
