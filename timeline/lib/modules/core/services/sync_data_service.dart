import 'dart:async';
import 'dart:developer';

import 'package:timeline/modules/core/services/api_service.dart';
import 'package:timeline/modules/core/services/database_service.dart';

class SyncDataService {
  Duration syncPeriod;
  DatabaseService? database;
  ApiService? api;

  SyncDataService({this.database, this.api, required this.syncPeriod}) {
    Timer.periodic(syncPeriod, (_) async {
      if (database == null || api == null) return;

      // this is not perfect, but good enough for now
      var newData = database?.getCurrentLocations() ?? [];
      try {
        await api
            ?.updateData(newData)
            .then((value) => database?.clearLocations());
      } catch (_) {
        log('Failed to sync to database');
      }
    });
  }
}
