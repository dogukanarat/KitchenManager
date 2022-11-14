import axios from 'axios'
import { API_URL } from '../Constants'

const api = axios.create({
    baseURL: API_URL,
})

export const postOrders = (content) => api.post('/v1/orders', content)

export const getOrders = () => api.get('/v1/orders')

const apis = {
    postOrders,
    getOrders,
}

export default apis