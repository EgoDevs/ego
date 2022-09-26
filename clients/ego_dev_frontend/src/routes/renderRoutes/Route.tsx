/* eslint-disable @typescript-eslint/strict-boolean-expressions */
import React from 'react';
import { __RouterContext as RouterContext } from 'react-router';

export default function Route(props: any) {
  return (
    <RouterContext.Consumer>
      {(context: any) => {
        const location = props.location || context.location;
        const match = props.computedMatch;
        const newProps = { ...context, location, match };
        const { render } = props;

        return (
          <RouterContext.Provider value={newProps}>
            {newProps.match
              ? render({
                  ...props.layoutProps,
                  ...newProps,
                })
              : null}
          </RouterContext.Provider>
        );
      }}
    </RouterContext.Consumer>
  );
}
