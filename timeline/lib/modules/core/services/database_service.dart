import 'package:flutter/material.dart';
import 'package:hive/hive.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:timeline/modules/core/model/location_entry.dart';

class DatabaseService extends ChangeNotifier {
  List<LocationEntry> entries = [];
  Box<LocationEntry> locationBox;
  Box<dynamic> utilBox;

  DatabaseService({required this.locationBox, required this.utilBox}) {
    locationBox.listenable().addListener(() {
      entries = locationBox.values.toList();
      notifyListeners();
    });
  }

  void addLocation(LocationEntry location) {
    locationBox.add(location);
  }

  List<LocationEntry> getCurrentLocations() {
    return locationBox.values.toList();
  }

  Future<void> clearLocations() async {
    await locationBox.clear();
  }
}
