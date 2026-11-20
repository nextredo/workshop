#! /usr/bin/python3

# Info:
# https://community.frame.work/t/solved-clear-fingerprint-reader-storage-fingerprint-reader-problems/22951
# https://gitlab.freedesktop.org/libfprint/libfprint/-/work_items/415#note_1063158

import gi
gi.require_version('FPrint', '2.0')
from gi.repository import FPrint

ctx = FPrint.Context()

for dev in ctx.get_devices():
    print(dev)
    print(dev.get_driver())
    print(dev.props.device_id);

    dev.open_sync()
    dev.clear_storage_sync()

    # prints = dev.list_prints_sync()
    # print("num prints stored: %d" % len(prints))
    # for p in prints:
    #     print('deleting print:')
    #     date = p.props.enroll_date
    #     print('    %04d-%02d-%02d valid: %d' % (date.get_year(), date.get_month(), date.get_day(), date.valid()))
    #     print('    ' + str(p.props.finger))
    #     print('    ' + str(p.props.username))
    #     print('    ' + str(p.props.description))
    #
    #     dev.delete_print_sync(p)
    #
    #     print('deleted')

    dev.close_sync()
