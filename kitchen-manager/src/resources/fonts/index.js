import {createGlobalStyle} from 'styled-components'

import GothamBlack from '../GothamBlack.ttf';
import GothamBold from '../GothamBold.ttf';

export default createGlobalStyle`
    @font-face {
        font-family: 'Gotham';
        src: url(${GothamBlack}) format('ttf');
        font-weight: normal;
        font-style: normal;
        font-display: auto;
    }
`;