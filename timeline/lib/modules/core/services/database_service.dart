import 'package:hive/hive.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:timeline/modules/core/model/location_entry.dart';

const String boxName = 'location';

class DatabaseService {
  addLocation(LocationEntry location) {
    Hive.box(boxName).add(location);
  }
}
