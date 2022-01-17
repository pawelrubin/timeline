import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:google_maps_flutter/google_maps_flutter.dart';
import 'package:hive/hive.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:provider/provider.dart';
import 'package:firebase_core/firebase_core.dart';
import 'package:timeline/modules/core/config/services.dart';
import 'package:timeline/modules/core/model/location_entry.dart';
import 'package:timeline/modules/core/services/api_service.dart';
import 'package:timeline/modules/core/services/database_service.dart';
import 'package:timeline/modules/core/services/location_service.dart';
import 'package:timeline/modules/core/services/sync_data_service.dart';
import 'package:timeline/modules/login/views/splash_view.dart';
import 'modules/core/services/authentication_service.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  if (defaultTargetPlatform == TargetPlatform.android) {
    AndroidGoogleMapsFlutter.useAndroidViewSurface = true;
  }

  // await async services setup
  await Future.wait(
      [dotenv.load(), Firebase.initializeApp(), Hive.initFlutter()]);

  // read values from env file
  var apiBaseUrl = dotenv.env['API_BASE_URL'] ?? '';
  var syncInterval =
      Duration(seconds: int.parse(dotenv.env['SYNC_INTERVAL'] ?? '10'));

  // setup hive
  Hive.registerAdapter(LocationEntryAdapter());
  var locationBox = await Hive.openBox<LocationEntry>(locationBoxName);
  var utilBox = await Hive.openBox(utilBoxName);

  runApp(MultiProvider(
    providers: [
      ChangeNotifierProvider(
          create: (_) =>
              DatabaseService(locationBox: locationBox, utilBox: utilBox)),
      Provider(create: (_) => AuthenticationService()),
      Provider(
        create: (_) => ApiService(apiBaseUrl: apiBaseUrl),
      ),
      ProxyProvider<DatabaseService, LocationService>(
        update: (context, database, location) =>
            LocationService(database: database),
        create: (_) => LocationService(database: null),
        lazy: false,
      ),
      ProxyProvider2<DatabaseService, ApiService, SyncDataService>(
        update: (_, db, api, __) =>
            SyncDataService(syncPeriod: syncInterval, database: db, api: api),
        create: (_) => SyncDataService(syncPeriod: syncInterval),
        lazy: false,
        dispose: (_, syncService) => syncService.timer.cancel(),
      )
    ],
    child: const TimelineApp(),
  ));
}

class TimelineApp extends StatelessWidget {
  const TimelineApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Timeline',
      theme:
          ThemeData(primarySwatch: Colors.amber, brightness: Brightness.dark),
      home: const SplashView(),
    );
  }
}
