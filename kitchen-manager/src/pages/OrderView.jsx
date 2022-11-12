import React, { Component } from 'react'
// import Api from '../api'
import ReactPlayer from 'react-player'
import Styled from 'styled-components'
import BgBlackImage from '../resources/bg-black.png'
import RoofYellowImage from '../resources/roof-yellow.png'
import RoofBlackImage from '../resources/roof-black.png'


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
    background-image: url(${RoofBlackImage});
    background-size: 50px 50px;
    background-repeat: no-repeat;
`

class OrderView extends Component {
    constructor(props) {
        super(props)
        this.state = {
            orders: [],
            isLoading: false,
        }
    }

    componentDidMount = async () => {
        this.setState({ isLoading: true })

        var orders = []

        // fill orders with index 0 to 9
        for (var i = 1; i < 9; i++) {
            orders.push(i * 123)
        }

        this.setState({ orders: orders, isLoading: false })
    }

    render() {
        const { orders } = this.state

        return (
            <Wrapper>
                <VideoCard>
                    <ReactPlayer 
                    url='https://www.youtube.com/watch?v=ysz5S6PUM-U'
                    config={{
                        youtube: {
                          playerVars: { showinfo: 0 }
                        }
                      }}
                    width='100%'
                    height='100%'
                    />
                </VideoCard>

                {orders.map((order, key) => (
                    <OrderCard key={key}>
                        <OrderInfo>
                            <OrderId>{order}</OrderId>
                            <OrderLocation>1</OrderLocation>
                        </OrderInfo>
                    </OrderCard>
                ))}
            </Wrapper>
        )
    }
}

export default OrderView