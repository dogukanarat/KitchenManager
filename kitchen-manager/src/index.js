import ReactDOM from 'react-dom/client';
import React from 'react'
import Styled from 'styled-components'
import { BrowserRouter, Routes, Route } from 'react-router-dom'
import { OrderView, OrderManager } from './pages'
import GlobalFonts from './resources/fonts'
import 'bootstrap/dist/css/bootstrap.min.css'

const BodyContainer = Styled.main.attrs({
})`
  display: flex;
  justify-content: center;
  background-color: #000;
  height: 100vh;
`

const NoPage = () => <h1><center>No Page!</center></h1>

function Body() {
  return (
    <BodyContainer>
      <Routes>
        <Route index element={<OrderView />} />
        <Route path="/order/view" element={<OrderView />} />
        <Route path="/order/manager" element={<OrderManager />} />
        <Route path="*" element={<NoPage />} />
      </Routes>
    </BodyContainer>
  )
}

export default function App() {
  return (
    <BrowserRouter>
      <GlobalFonts />
      <Body />
    </BrowserRouter>
  )
}

const rootElement = document.getElementById('root');

const root = ReactDOM.createRoot(rootElement);

root.render(<App />);