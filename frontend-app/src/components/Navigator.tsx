// [Module] React
import * as React from 'react'

// [Module] React Navigation
import { BaseNavigationContainer } from '@react-navigation/core'
import { stackNavigatorFactory } from 'react-nativescript-navigation'

// [Module] Project
import { toMonthString } from '../utils'
import { GameService } from '../services'
import { MainStackParamList } from './NavigationParamList'
import { HomeScreen } from './HomeScreen'
import { DateViewScreen } from './DateViewScreen'

const StackNavigator = stackNavigatorFactory()

export const mainStackNavigator = () => {
  const gameDate = (dateId: string) => {
    const date = GameService.getRecordById(dateId).date;

    if (date === undefined) {
      return 'Error getting date';
    }

    return `${toMonthString(date.getUTCMonth())} ${date.getUTCDate()}, ${date.getUTCFullYear()}`;
  };

  return (
    <BaseNavigationContainer>
      <StackNavigator.Navigator
        initialRouteName='Home'
        screenOptions={{
          headerShown: true
        }}
      >
        <StackNavigator.Screen
          name='Home'
          options={{
            title: 'Bowling Tracker'
          }}
          component={HomeScreen}
        />
        <StackNavigator.Screen
          name='DateView'
          options={({ route }) => ({
            title: gameDate((route.params as MainStackParamList['DateView']).dateId)
          })}
          component={DateViewScreen}
        />
      </StackNavigator.Navigator>
    </BaseNavigationContainer>
  )
}
