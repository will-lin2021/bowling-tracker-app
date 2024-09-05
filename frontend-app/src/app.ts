// [Module] React
import * as React from 'react';

// [Module] NativeScript
import * as ReactNativeScript from 'react-nativescript';
import { mainStackNavigator as AppContainer } from './components/Navigator';

// [Module] Project
import { GameService } from './services';

// Controls react-nativescript log verbosity.
// - true: all logs;
// - false: only error logs.
Object.defineProperty(global, '__DEV__', { value: false });

// App Init Stuff
GameService.init();

// In NativeScript, the app.ts file is the entry point to your application. You
// can use this file to perform app-level initialization, but the primary
// purpose of the file is to pass control to the app’s first module.

ReactNativeScript.start(React.createElement(AppContainer, {}, null));

// Do not place any code after the application has been started as it will not
// be executed on iOS.
