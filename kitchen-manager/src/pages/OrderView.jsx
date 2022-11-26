import React, { Component } from 'react'
import Api from '../api'
import { API_URL } from '../Constants'
import ReactPlayer from 'react-player'
import Styled from 'styled-components'
import BgBlackImage from '../resources/bg-black.png'
import ShelfBlackImage from '../resources/shelf-black.png'
import BgYellowImage from '../resources/bg-yellow.png'
import ShelfYellowImage from '../resources/shelf-yellow.png'

const Wrapper = Styled.div.attrs({
})`
    width: 1200px;
    height: 800px;
`

const CardItem = Styled.div.attrs({
})`
    float: left;
    text-align: center;
    color: #fff;
    justify-content: center;
    display: flex;
    flex-direction: column;
    font-family: 'Arial';
`

const VideoCard = Styled(CardItem)`
    background-color: #c0c0c0;
    height: 590px;
    width: 890px;
    margin: 5px;
    background-color: #000;
    padding: 10px;
`

const OrderCard = Styled(CardItem)`
    height: 140px;
    width: 290px;
    margin: 5px;
    background-image: url(${BgBlackImage});
    background-size: 290px 140px;
    background-repeat: no-repeat;
`

const OrderInfo = Styled.div.attrs({
})`
    align-self: center;
`

const OrderInfoItem = Styled.div.attrs({
})`
    float: left;
    margin-left: 10px;
    margin-right: 10px;
    height: 50px;
    justify-content: center;
    display: flex;
    flex-direction: column;
`

const OrderId = Styled(OrderInfoItem)`
    font-size: 50px;
    color: yellow;
`

const OrderLocation = Styled(OrderInfoItem)`
    float: left;
    width: 50px;
    color: #fff;
    background-image: url(${ShelfBlackImage});
    background-size: 50px 50px;
    background-repeat: no-repeat;
`

class OrderView extends Component {
    constructor(props) {
        super(props)
        this.state = {
            orders: [],
            videos: [],
            currentVideoIndex: 0,
            isLoading: false,
        }

        document.title = 'Siparis Akisi';

        let result = require.context('../resources/videos/', false, /\.(mp4)$/);

        result.keys().forEach((key) => {
            this.state.videos.push(
                {
                    id: key,
                    url: result(key)
                });
        });
    }

    componentDidMount = async () => {
        this.setState({ isLoading: true })

        const eventSource = API_URL + "/v1/orders/events/create"

        const events = new EventSource(eventSource);

        events.addEventListener("order_create", event => {
            let eventData = event.data

            if (eventData.length > 0) {
                let jsonData = JSON.parse(eventData)
                this.addNewOrder(jsonData.data)
            }

        })
    }

    addNewOrder = (order) => {
        this.setState({ orders: [order, ...this.state.orders] }, () => {

            let newOrders = this.state.orders

            if (newOrders.length > 8) {
                newOrders.pop()
            }

            this.setState({ orders: newOrders })

        })
    }

    render() {
        const { orders } = this.state

        return (
            <Wrapper>
                <meta name="mobile-web-app-capable" content="yes"></meta>
                <VideoCard>
                    <ReactPlayer
                        url={this.state.videos[this.state.currentVideoIndex].url}
                        config={{
                            youtube: {
                                playerVars: { showinfo: 0 }
                            }

                        }}
                        width='100%'
                        height='100%'
                        playing={true}
                        controls={false}
                        onEnded={() => {
                            let newIndex = this.state.currentVideoIndex + 1

                            if (newIndex >= this.state.videos.length) {
                                newIndex = 0
                            }

                            this.setState({ currentVideoIndex: newIndex })
                        }}
                    />
                </VideoCard>

                {orders.map((order, key) => (
                    <OrderCard key={key}>
                        <OrderInfo>
                            <OrderId>{order.id}</OrderId>
                            <OrderLocation>{order.shelf}</OrderLocation>
                        </OrderInfo>
                    </OrderCard>
                ))}
            </Wrapper>
        )
    }
}

export default OrderView