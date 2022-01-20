import 'dart:async';
import 'dart:developer';

import 'package:timeline/modules/core/services/api_service.dart';
import 'package:timeline/modules/core/services/database_service.dart';

class SyncDataService {
  Duration syncPeriod;
  DatabaseService? database;
  ApiService? api;
  late Timer timer;
  var _isUpdating = false;

  SyncDataService({this.database, this.api, required this.syncPeriod}) {
    timer = Timer.periodic(syncPeriod, (_) async {
      if (_isUpdating || database == null || api == null) return;
      _isUpdating = true;

      // this is not perfect, but good enough for now
      var newData = database?.getCurrentLocations() ?? [];
      try {
        await api
            ?.updateData(newData)
            .then((value) => database?.clearLocations());
      } catch (e) {
        log('Failed to sync to database');
      }
      _isUpdating = false;
    });
  }
}
