import React, { Component } from 'react'
import Api from '../api'
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

.form-signin button {
    margin-bottom: 20px;
  }

`

class OrderManager extends Component {
    constructor(props) {
        super(props)
        this.state = {
            isLoading: false,
            orderId: '',
            orderShelf: ''
        }

        this.onOrderIdChange = this.onOrderIdChange.bind(this);
        this.onOrderShelfChange = this.onOrderShelfChange.bind(this);
        this.onClickSubmit = this.onClickSubmit.bind(this);

        document.title = 'Siparis Yonetimi';
    }

    componentDidMount = async () => {
        this.setState({ isLoading: true })

        this.setState({ isLoading: false })
    }

    addNewOrder = async (order) => {

    }

    onOrderIdChange = async (event) => {
        this.setState({ orderId: event.target.value })
    }

    onOrderShelfChange = async (event) => {
        this.setState({ orderShelf: event.target.value })
    }

    onClickSubmit = async (event) => {

        event.preventDefault()

        const { orderId, orderShelf } = this.state

        this.setState({ isSuccess: false, isMessageVisible: false })

        const payload = {
            "id": parseInt(orderId),
            "shelf": parseInt(orderShelf)
        }

        let result = []

        try {
            result = await Api.postOrders(payload)

            if (result.status === 200) {
                this.setState({ isSuccess: true })
            }
            else {
                this.setState({ isSuccess: false })
            }
        } catch (error) {
            this.setState({ isSuccess: false })
        }

        this.setState({ orderId: '', orderShelf: '', isMessageVisible: true })

        setTimeout(() => {
            this.setIsMessageVisible(false);
        }, 3000);

        event.stopPropagation();
        event.nativeEvent.stopPropagation();
    }

    setIsMessageVisible = (value) => {
        this.setState({ isMessageVisible: value })
    }

    render() {
        return (
            <Wrapper>
                <main className="form-signin w-400 m-auto">
                    <form>
                        <h1 className="h3 mb-3 fw-normal text-white">Siparis Guncelle</h1>

                        <div className="form-floating">
                            <input type="text" className="form-control" id="floatingInput"
                                value={this.state.orderId}
                                placeholder="Siparis Id"
                                onChange={(event) => { this.onOrderIdChange(event) }} />
                            <label htmlFor="floatingInput">Siparis No</label>
                        </div>

                        <div className="form-floating">
                            <input type="text" className="form-control" id="floatingInput"
                                value={this.state.orderShelf}
                                placeholder="Raf No"
                                onChange={(event) => { this.onOrderShelfChange(event) }} />
                            <label htmlFor="floatingInput">Siparis Raf</label>
                        </div>

                        <button className="w-100 btn btn-lg btn-primary" type="submit"
                            onClick={(event) => { this.onClickSubmit(event) }}>Hazir</button>
                    </form>


                    {this.state.isMessageVisible && !this.state.isSuccess ?
                        <div className="alert alert-danger" role="alert">
                            Bir hata olustu! Tekrar deneyin...
                        </div>
                        : null
                    }

                    {this.state.isMessageVisible && this.state.isSuccess ?
                        <div className="alert alert-success" role="alert">
                            Siparis basariyla eklendi!
                        </div>
                        : null
                    }

                </main>

            </Wrapper>
        )
    }
}

export default OrderManager