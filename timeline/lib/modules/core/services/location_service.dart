import 'package:flutter_background_geolocation/flutter_background_geolocation.dart'
    as bg;
import 'package:timeline/modules/core/model/location_entry.dart';
import 'package:timeline/modules/core/services/database_service.dart';

class LocationService {
  DatabaseService? database;

  LocationService({this.database}) {
    bg.BackgroundGeolocation.onLocation((location) => {
          database?.addLocation(LocationEntry(
              dateTime: DateTime.parse(location.timestamp),
              longitude: location.coords.longitude,
              altitude: location.coords.altitude,
              latitude: location.coords.latitude))
        });

    bg.BackgroundGeolocation.ready(bg.Config(
            desiredAccuracy: bg.Config.DESIRED_ACCURACY_HIGH,
            distanceFilter: 10.0,
            stopOnTerminate: false,
            startOnBoot: true,
            debug: true,
            logLevel: bg.Config.LOG_LEVEL_VERBOSE))
        .then((bgState) => {
              if (!bgState.enabled) {bg.BackgroundGeolocation.start()}
            });
  }
}
