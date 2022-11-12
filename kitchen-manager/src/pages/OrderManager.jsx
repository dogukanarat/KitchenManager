import React, { Component } from 'react'
import Styled from 'styled-components'

const Wrapper = Styled.div.attrs({
})`
html,
body {
  height: 100%;
}

body {
  display: flex;
  align-items: center;
  padding-top: 40px;
  padding-bottom: 40px;
  background-color: #f5f5f5;
}

.form-signin {
  width: 560px;
  padding: 15px;
}

.form-signin .form-floating:focus-within {
  z-index: 2;
}

.form-signin input[type="text"] {
  margin-bottom: 10px;
}

`

class OrderManager extends Component {
    constructor(props) {
        super(props)
        this.state = {
            isLoading: false,
        }
    }

    componentDidMount = async () => {
        this.setState({ isLoading: true })

        this.setState({ isLoading: false })
    }

    render() {
        return (
            <Wrapper>
                <main className="form-signin w-400 m-auto ">
                    <form>
                        <img className="mb-4" src="/docs/5.2/assets/brand/bootstrap-logo.svg" alt="" width="150" height="57" />

                        <h1 className="h3 mb-3 fw-normal text-white">Siparis Guncelle</h1>

                        <div className="form-floating">
                            <input type="text" className="form-control" id="floatingInput" placeholder="123" />
                            <label htmlFor="floatingInput">Siparis No</label>
                        </div>

                        <div className="form-floating">
                            <input type="text" className="form-control" id="floatingInput" placeholder="123" />
                            <label htmlFor="floatingInput">Siparis Raf</label>
                        </div>

                        <button className="w-100 btn btn-lg btn-primary" type="submit">Hazir</button>
                    </form>
                </main>

            </Wrapper>
        )
    }
}

export default OrderManager