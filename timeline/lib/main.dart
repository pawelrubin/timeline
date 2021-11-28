import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:firebase_core/firebase_core.dart';
import 'package:timeline/modules/core/services/api_service.dart';
import 'package:timeline/modules/core/services/database_service.dart';
import 'package:timeline/modules/core/services/location_service.dart';
import 'package:timeline/modules/login/views/splash_view.dart';
import 'modules/core/services/authentication_service.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  await Future.wait([dotenv.load(), Firebase.initializeApp()]);

  var apiBaseUrl = dotenv.env['API_BASE_URL'] ?? '';

  runApp(MultiProvider(
    providers: [
      Provider(create: (_) => DatabaseService()),
      Provider(create: (_) => AuthenticationService()),
      ProxyProvider<DatabaseService, LocationService>(
        update: (context, database, location) =>
            LocationService(database: database),
        create: (_) => LocationService(database: null),
        lazy: false,
      ),
      ProxyProvider<DatabaseService, ApiService>(
          update: (context, database, api) =>
              ApiService(apiBaseUrl: apiBaseUrl, database: database),
          create: (_) => ApiService(apiBaseUrl: apiBaseUrl, database: null))
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
