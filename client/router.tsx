import * as React from 'react';
import { Router, Route } from 'dva/router';
import Home from './routes/Home';

export default function({ history }) {
  return (
    <Router history={history}>
      <Route path="/" component={Home} />
    </Router>
  );
}
