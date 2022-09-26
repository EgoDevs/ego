import { BrowserRouter as Router, Switch, Route } from 'react-router-dom';
import renderRoutes from './renderRoutes/renderRoutes';
import routes from './routes';

const RouterContainer = () => {
  return (
    <Router>
      <Switch>
        {/* <Route exact path="/">
        </Route> */}
        <Route path="*">
          {renderRoutes({
            routes: routes,
          })}
        </Route>
      </Switch>
    </Router>
  );
};

export default RouterContainer;
